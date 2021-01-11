require 'erb'
require 'yaml'

template = ERB.new(File.read("calculate_scores.rs.erb"), nil, '-')
alphabets = YAML.load_file("./latin.yml")

pp alphabets
puts puts puts
puts template.result

#File.open(LANG_OUTPUT, 'w') { |out| out.write(template.result) }
