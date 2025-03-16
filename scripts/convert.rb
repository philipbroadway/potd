require 'json'

# Check for input file argument
if ARGV.length != 1
  puts "Usage: ruby transform_json.rb <input_file.json>"
  exit 1
end

input_file = ARGV[0]

# Read and parse the input JSON file
begin
  json_data = File.read(input_file)
  data = JSON.parse(input_json)
rescue Errno::ENOENT
  puts "Error: File not found - #{input_file}"
  exit 1
rescue JSON::ParserError
  puts "Error: Invalid JSON format in #{input_file}"
  exit 1
end

# Transform the data
def transform_structure(data)
  transformed = {}
  
  data.each do |key, value|
    transformed[key] = { 'chapter' => {} }
    
    value['chapters'].each do |chapter|
      chapter_number = chapter['number']
      transformed[key]['chapter'][chapter_number] = { 'verse' => {} }
      
      chapter['verses'].each do |verse|
        transformed[key]['chapter'][chapter_number]['verse'][verse['number']] = verse['text']
      end
    end
  end
  
  transformed
end

flattened_data = transform_structure(json_data)

# Save the transformed data as JSON
output_path = 'output.json'
File.write(output_path, JSON.pretty_generate(flattened_data))

puts "Transformed JSON saved to #{output_path}"
