require "json"
require "erb"
require "pp"

DATA_FILE = File.expand_path("../data.json", __FILE__)
TEMPLATE_FILE = File.expand_path("../lang.rs.erb", __FILE__)

TARGET_FILE = File.expand_path("../../src/lang.rs", __FILE__)


SUPPORTED_LANGS = %w(
  epo
  cmn
  kor
  spa
  eng
  hin
  arb
  rus
  ben
  por
  fra
  deu
  ukr
  kat
  jpn
  heb
  ydd
  pol
  amh
  tir
  jav
  ita
  nob
  nno
  dan
  swe
  fin
  tur
  nld
  hun
  ces
  ell
  bul
  bel
  mar
  kan
  ron
)

class Script
  attr_reader :name, :langs

  def initialize(name)
    @name = name
    @langs = []
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
  attr_reader :scripts, :lang_codes

  def initialize(scripts, lang_codes)
    @scripts = scripts
    @lang_codes = lang_codes
  end

  def context
    binding
  end
end

def parse_data_file
  scripts = []

  data = JSON.parse(File.read(DATA_FILE))
  data.each do |script, langs_data|
    script = Script.new(script)
    langs_data.each do |lang_code, trigrams|
      if SUPPORTED_LANGS.include?(lang_code)
        script.langs << Lang.new(lang_code, trigrams.split("|"))
      end
    end
    scripts << script unless script.langs.empty?
  end

  scripts
end

scripts = parse_data_file
context = Context.new(scripts, SUPPORTED_LANGS.sort)

template = File.read(TEMPLATE_FILE)
renderer = ERB.new(template, nil, ">")
content = renderer.result(context.context)
File.write(TARGET_FILE, content)
