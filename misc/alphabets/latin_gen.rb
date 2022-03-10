require 'yaml'

def normalize_alphabet(alphabet)
  alphabet.downcase
    .chars
    .uniq
    .reject { |c| c == " " }
    .sort
    .join("")
end

def load_alphabets
  alphabets = {}
  data = YAML.load_file("./raw_latin.yml")

  latin_based = data["latin_based"]
  base = latin_based.delete("base")

  latin_based.each do |code, alphabet|
    alphabet = base + alphabet
    alphabet = normalize_alphabet(alphabet)
    alphabets[code] = alphabet
  end

  data["others"].each do |code, alphabet|
    alphabets[code] = normalize_alphabet(alphabet)
  end

  alphabets.sort_unstable_by {|k, _| k }.to_h
end


alphabets = load_alphabets

puts alphabets.to_yaml

