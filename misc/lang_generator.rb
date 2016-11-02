require "json"
require "erb"
require "pp"

DATA_FILE = File.expand_path("../data.json", __FILE__)
TEMPLATE_FILE = File.expand_path("../lang.rs.erb", __FILE__)

TARGET_FILE = File.expand_path("../../src/lang.rs", __FILE__)

class Lang
  attr_reader :code, :trigrams, :script

  SIZE = 300

  def initialize(code, script, trigrams)
    @code = code
    @script = script
    @trigrams = trigrams
    @use_script = false

    if trigrams.size != SIZE
      raise "Language #{code}, has #{trigrams.size} trigrams, instead of #{SIZE}"
    end
  end

  def enum
    if @use_script
      "#{code.capitalize}#{script.capitalize}"
    else
      code.capitalize
    end
  end

  def use_script!
    @use_script = true
  end
end

class Render
  attr_reader :langs

  def initialize(langs)
    @langs = langs
  end
end

def parse_data_file
  langs = []
  data = JSON.parse(File.read(DATA_FILE))
  data.each do |script, langs_data|
    langs_data.each do |lang_code, trigrams|
      langs << Lang.new(lang_code, script, trigrams.split("|"))
    end
  end
  langs
end

langs = parse_data_file
codes = langs.map(&:code)

langs.each do |lang|
  if codes.count(lang.code) > 1
    lang.use_script!
  end
end

template = File.read(TEMPLATE_FILE)
renderer = ERB.new(template, nil, ">")
content = renderer.result(binding)

File.write(TARGET_FILE, content)
