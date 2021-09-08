def is_digit?(c)
    c.match(/[[:digit:]]/)
end

def tokenize(input)
    input = input.gsub(/\s+/, "")

    tokens = Array.new
    number = nil
    input.split('').each do |c|
        if is_digit?(c)
            unless number
                number = ""
            end
            number.concat(c)
        else
            if number
                tokens.append(number.to_i)
                number = nil
            end

            raise ArgumentError, "Invalid character: #{c}" unless "+-/*()".include? c 
            tokens.append(c)
        end
    end

    if number
        tokens.append(number.to_i)
        number = nil
    end

    tokens
end