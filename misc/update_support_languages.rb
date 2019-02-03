# Updates README with list of supported languages

require "csv"
require "erb"
require "json"
require "pp"
require "pry"

LIST_FILE = File.expand_path("../supported_languages.csv", __FILE__)
JSON_FILE = File.expand_path("../data.json", __FILE__)
README_FILE = File.expand_path("../../README.md", __FILE__)
LANG_TEMPLATE_FILE = File.expand_path("../lang.rs.erb", __FILE__)
LANG_OUTPUT = File.expand_path("../../src/lang.rs", __FILE__)
TRIGRAM_COUNT = 300

class Lang
  attr_reader :code, :eng_name, :name, :native_speakers, :script, :trigrams

  def initialize(code, eng_name, name, script, trigrams, native_speakers = nil)
    @code = code || raise("Missing code")
    @eng_name = eng_name || raise("Missing eng_name")
    @name = name || eng_name || raise("Missing name")
    @script = script || raise("Missing script")
    @trigrams = trigrams || raise("Missing trigrams")
    @native_speakers = native_speakers
  end

  def self.load
    langs = []
    json = JSON.parse(File.read(JSON_FILE))
    rows = CSV.read(LIST_FILE, headers: true).each
    json.each do |script, languages|
      languages.each do |lang, trigrams|
        row = rows.find { |r| r["code"] && r["code"] == lang }
        if row
          langs << Lang.new(row["code"], row["eng_name"], row["name"], script, trigrams.split('|'), row["native_speakers"])
        end
      end
    end
    return langs, json
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

template = ERB.new(File.read(LANG_TEMPLATE_FILE))
File.open(LANG_OUTPUT, 'w') { |out| out.write(template.result) }
`cargo fmt` # Call cargo fmt to clean the generated code

readme = File.read(README_FILE)

readme.gsub!(/\| Language .+\|\n/m, table.to_s)

File.write(README_FILE, readme)
