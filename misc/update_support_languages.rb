# Updates SUPPORTED_LANGUAGES.md with list of supported languages

require "csv"
require "erb"
require "json"
require "pp"

LIST_FILE = File.expand_path("../supported_languages.csv", __FILE__)
JSON_FILE = File.expand_path("../data.json", __FILE__)
LANG_TEMPLATE_FILE = File.expand_path("../lang.rs.erb", __FILE__)
LANG_OUTPUT = File.expand_path("../../src/lang.rs", __FILE__)

TRIGRAM_PROFILES_TEMPLATE_FILE = File.expand_path("../trigram_profiles.rs.erb", __FILE__)
TRIGRAM_PROFILES_TARGET = File.expand_path("../../src/trigrams/profiles.rs", __FILE__)

TRIGRAM_COUNT = 300

OUTPUT_FILE = File.expand_path("../../SUPPORTED_LANGUAGES.md", __FILE__)

IGNORE_LANGS = {
  # Do not generate cyrillic trigrams for Turkmen and Azerbaijani
  "Cyrillic" => ["tuk", "aze"]
}

class Lang
  attr_reader :code, :eng_name, :name, :native_speakers, :script, :trigrams

  def initialize(code, eng_name, name, script, trigrams, native_speakers = nil)
    @code = code || raise("Missing code")
    @eng_name = eng_name || raise("Missing eng_name")
    @name = name || eng_name || raise("Missing name")
    @native_speakers = native_speakers
  end

  def self.load
    langs = []
    rows = CSV.read(LIST_FILE, headers: true).each
    rows.each do |row|
      if !langs.any? { |l| l.code == row["code"] }
        langs << Lang.new(row["code"], row["eng_name"], row["name"], "", [], row["native_speakers"])
      end
    end

    scripts = {}
    json = JSON.parse(File.read(JSON_FILE))
    json.each do |script, languages|
      if !scripts[script]
        scripts[script] = []
      end
      ignore_langs = IGNORE_LANGS[script] || []
      languages.each do |lang, trigrams|
        next if ignore_langs.include?(lang)

        info = langs.find { |l| l.code == lang }
        if info
          scripts[script] << {
            code: lang,
            script: script,
            trigrams: trigrams.split('|')
          }
        end
      end
    end

    # Filter out scripts with only one language
    scripts.select! {|script, langs| langs.size > 1 }

    return langs, scripts
  end
end

class MarkdownTable
  def initialize(headers)
    @headers = headers
    @rows = []
  end

  def add(row)
    @rows << row
  end

  def to_s
    widths = []
    @headers.each_with_index do |header, i|
      header_size = header.to_s.size
      cell_size = @rows.map { |r| r[i].to_s.size }.max
      widths[i] = [header_size, cell_size].max
    end

    output = "|"
    @headers.each_with_index do |h, i|
      width = widths[i]
      output << " " << h.ljust(width) << " |"
    end
    output << "\n"

    output << "|"
    widths.each do |w|
      output << " " << ("-" * w) << " |"
    end
    output << "\n"

    @rows.each do |row|
      output << "|"
      row.each_with_index do |item, i|
        width = widths[i]
        output << " " << item.ljust(width) << " |"
      end
      output << "\n"
    end

    output
  end
end


langs, scripts = Lang.load

table = MarkdownTable.new(["Language", "ISO 639-3", "Enum"])
langs.each do |lang|
  table.add([lang.eng_name, lang.code, "`Lang::#{lang.code.capitalize}`"])
end
supported_langs_table = File.read(OUTPUT_FILE)
supported_langs_table.gsub!(/\| Language .+\|\n/m, table.to_s)
File.write(OUTPUT_FILE, supported_langs_table)

template = ERB.new(File.read(LANG_TEMPLATE_FILE))
File.open(LANG_OUTPUT, 'w') { |out| out.write(template.result) }

template = ERB.new(File.read(TRIGRAM_PROFILES_TEMPLATE_FILE))
File.open(TRIGRAM_PROFILES_TARGET, 'w') { |out| out.write(template.result) }

`cargo fmt` # Call cargo fmt to clean the generated code

scripts.each do |script, data|
  puts script
  data.each do |lang_data|
    lang = langs.find { |l| lang_data[:code] == l.code }
    puts "[ ] #{lang.code} (#{lang.eng_name})"
  end
  puts
end
