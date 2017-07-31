require "json"
require "erb"
require "csv"
require "pp"

DATA_FILE = File.expand_path("../data.json", __FILE__)
TEMPLATE_FILE = File.expand_path("../lang.rs.erb", __FILE__)
LIST_FILE = File.expand_path("../supported_laguages.csv", __FILE__)
TARGET_FILE = File.expand_path("../../src/lang.rs", __FILE__)

class Script
  attr_reader :name, :langs

  def initialize(name)
    @name = name
    @langs = []
  end
end

class SupportedLang
  attr_reader :code, :eng_name

  def initialize(code, eng_name)
    @code = code
    @eng_name = eng_name
  end
end

class Lang
  attr_reader :code, :trigrams, :script

  SIZE = 300

  def initialize(code, trigrams)
    @code = code
    @script = script
    @trigrams = trigrams
    @use_script = false

    if trigrams.size != SIZE
      raise "Language #{code}, has #{trigrams.size} trigrams, instead of #{SIZE}"
    end
  end
end

class Context
  attr_reader :scripts, :supported_langs

  def initialize(scripts, supported_langs)
    @scripts = scripts
    @supported_langs = supported_langs
  end

  def context
    binding
  end
end


def load_supported_langs
  langs = []
  CSV.read(LIST_FILE, headers: true).each do |row|
    if row["code"] && row["eng_name"]
      langs << SupportedLang.new(row["code"], row["eng_name"])
    end
  end
  langs.sort {|a,b| a.code <=> b.code }
end

SUPPORTED_LANGS = load_supported_langs
SUPPORTED_LANG_CODES = SUPPORTED_LANGS.map(&:code)


def parse_data_file
  scripts = []

  data = JSON.parse(File.read(DATA_FILE))
  data.each do |script, langs_data|
    script = Script.new(script)
    langs_data.each do |lang_code, trigrams|
      if SUPPORTED_LANG_CODES.include?(lang_code)
        script.langs << Lang.new(lang_code, trigrams.split("|"))
      end
    end
    scripts << script unless script.langs.empty?
  end

  scripts
end

scripts = parse_data_file
context = Context.new(scripts, SUPPORTED_LANGS)

template = File.read(TEMPLATE_FILE)
renderer = ERB.new(template, nil, ">")
content = renderer.result(context.context)
File.write(TARGET_FILE, content)
