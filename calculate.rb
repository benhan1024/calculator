require "./lexer.rb"
require "./parser.rb"

def traverse(node)
    if node.instance_of? Integer
        return node
    else
        left = traverse(node.left)
        right = traverse(node.right)

        if node.op == '+'
            return left + right
        elsif node.op == '-'
            return left - right
        elsif node.op == '/'
            return left / right
        else # node.op == '*''
            return left * right
        end
    end 
end

def calculate(input)
    tokens = tokenize(input)
    tree = parse(tokens)
    traverse(tree)
end