class OpNode
    attr_reader :left, :op, :right

    def initialize(left, op, right)
        @left = left
        @op = op
        @right = right
    end
end

class Stack
    def initialize
        @nodes = Array.new
        @ops = Array.new
    end

    def push_node(node)
        @nodes.append(node)
    end

    def push_op(op)
        if @ops.length > 0
            top_op_prec = get_operator_precedence(@ops.last)
            this_op_prec = get_operator_precedence(op)
            if top_op_prec >= this_op_prec
                merge_nodes
            end
        end
        @ops.append(op)
    end

    def resolve_all()
        raise RuntimeError, "Cannot call resolve_all with empty node stack" unless @nodes.length > 0

        while @nodes.length > 1 do
            merge_nodes
        end

        return @nodes[0]
    end

    private

    def merge_nodes()
        right = @nodes.pop
        op = @ops.pop
        left = @nodes.pop

        @nodes.push(OpNode.new(left, op, right))
    end

    def get_operator_precedence(op)
        if '+-'.include? op
            return 1
        else # /*
            return 2
        end
    end
end

def is_operator?(op)
    "+-/*".include? op
end

def validate_token(last_token, this_token)
    if last_token == nil
        return
    elsif last_token.instance_of? Integer
        raise ArgumentError, "Syntax error. Operator must follow a number" unless is_operator? this_token
    elsif last_token == '('
        raise ArgumentError, "Number must follow '('" unless this_token.instance_of? Integer
    elsif last_token == ')'
        raise ArgumentError, "Operator must follow ')'" unless is_operator? this_token
    else # operator
        raise ArgumentError, "Number or '(' must follow operator" unless this_token.instance_of? Integer || this_token == '('
    end
end

def parse(token_array)
    stacks = [Stack.new]

    last_token = nil
    for token in token_array do
        validate_token(last_token, token)
        last_token = token

        if token.instance_of? Integer
            stacks.last.push_node(token)
        elsif token.instance_of? String
            if token == '('
                stacks.append(Stack.new)
            elsif token == ')'
                node = stacks.last.resolve_all
                stacks.pop
                stacks.last.push_node(node)
            else
                stacks.last.push_op(token)
            end
        else
            raise ArgumentError, "Invalid token: #{token}"
        end
    end

    stacks.last.resolve_all
end

def print_tree(tree)
    if tree.instance_of? OpNode
        print '('
        print_tree(tree.left)
        print ' '
        print tree.op
        print ' '
        print_tree(tree.right)
        print ')'
    else
        print tree
    end
end