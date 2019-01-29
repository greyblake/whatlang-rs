# Updates SUPPORTED_LANGUAGES.md with list of supported languages

require "csv"
require "pp"
require "pry"

LIST_FILE = File.expand_path("../supported_languages.csv", __FILE__)
OUTPUT_FILE = File.expand_path("../../SUPPORTED_LANGUAGES.md", __FILE__)

class Lang
  attr_reader :code, :eng_name

  def initialize(code, eng_name)
    @code = code || raise("Missing code")
    @eng_name = eng_name || raise("Missing eng_name")
  end

  def self.load
    langs = []
    CSV.read(LIST_FILE, headers: true).each do |row|
      langs << Lang.new(row["code"], row["eng_name"]) if row["code"]
    end
    langs
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


langs = Lang.load

table = MarkdownTable.new(["Language", "ISO 639-3", "Enum"])
langs.each do |lang|
  table.add([lang.eng_name, lang.code, "`Lang::#{lang.code.capitalize}`"])
end

readme = File.read(OUTPUT_FILE)

readme.gsub!(/\| Language .+\|\n/m, table.to_s)

File.write(OUTPUT_FILE, readme)
