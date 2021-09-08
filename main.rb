require "./calculate.rb"

if ARGV.length < 1 || ARGV[0].length < 1
    puts 0
else
    puts calculate(ARGV[0])
end