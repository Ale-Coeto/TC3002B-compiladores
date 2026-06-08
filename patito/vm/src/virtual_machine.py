import operator as op
from src.memory import Memory
from src.quad_codes import QuadCode
from src.logger import Logger

BINARY_OPS = {
    QuadCode.ADD: (op.add, lambda l, r: f"Adding: {l} + {r}"),
    QuadCode.SUB: (op.sub, lambda l, r: f"Subtracting: {l} - {r}"),
    QuadCode.MUL: (op.mul, lambda l, r: f"Multiplying: {l} * {r}"),
    QuadCode.DIV: (op.truediv, lambda l, r: f"Dividing: {l} / {r}"),
    QuadCode.LT:  (op.lt, lambda l, r: f"Comparing: {l} < {r}"),
    QuadCode.GT:  (op.gt, lambda l, r: f"Comparing: {l} > {r}"),
    QuadCode.EQ:  (op.eq, lambda l, r: f"Comparing: {l} == {r}"),
    QuadCode.NEQ: (op.ne, lambda l, r: f"Comparing: {l} != {r}"),
}

RED   = "\033[91m"
RESET = "\033[0m"

class VirtualMachine:
    def __init__(self, program=None, verbose=False):
        self.instruction_pointer = 0
        self.call_stack = []
        self.program = program
        self.functions = {func.name: func for func in program.functions} if program else {}
        self.memory = Memory(program.constants) if program else None
        self.logger = Logger(verbose)

    def errors(self, errors):
        for e in errors:
            print(f"{RED}[{e.type}]{RESET} {e.message}")

    def run(self):
        while True:
            curr_instruction = self.program.quads[self.instruction_pointer]
            curr_operator = curr_instruction.operator
            self.instruction_pointer += 1
            self.logger.header("Executing", f"{curr_operator.name} {curr_instruction.left_operand} {curr_instruction.right_operand} {curr_instruction.result}")
            message = ""

            if curr_operator == QuadCode.GOTO:
                self.instruction_pointer = curr_instruction.result
                message = f"Jumping to instruction {curr_instruction.result}"

            elif curr_operator == QuadCode.GOTOF:
                value = self.memory.get_value(curr_instruction.left_operand)
                if value is False:
                    self.instruction_pointer = curr_instruction.result
                    message = f"Condition is false, jumping to instruction {curr_instruction.result}"
                else:
                    message = "Condition is true, continuing to next instruction"

            elif curr_operator == QuadCode.PRINT:
                value = self.memory.get_value(curr_instruction.result)
                print(value)
                message = f"Printing value: {value}"

            elif curr_operator == QuadCode.ERA:
                self.memory.new_local_memory()
                message = f"Creating new local memory for function {curr_instruction.result}"

            elif curr_operator == QuadCode.PARAM:
                self.memory.set_param(curr_instruction.left_operand) 
                value = self.memory.get_value(curr_instruction.left_operand)
                message = f"Setting parameter #{curr_instruction.result+1} with value: {value}"

            elif curr_operator == QuadCode.GOSUB:
                self.memory.use_new_local_memory()
                self.call_stack.append(self.instruction_pointer)
                self.instruction_pointer = self.functions[curr_instruction.result].start_index
                message = f"Jumping to function {curr_instruction.result}, instruction {self.instruction_pointer}"

            elif curr_operator == QuadCode.RETURN:
                value = self.memory.get_value(curr_instruction.left_operand)
                self.memory.set_value(curr_instruction.result, value)
                message = f"Returning value: {value}"

            elif curr_operator == QuadCode.ENDFUNC:
                self.instruction_pointer = self.call_stack.pop()
                self.memory.release_local_memory()
                message = "Function ended, clearing memory"

            elif curr_operator == QuadCode.ASSIGN:
                value = self.memory.get_value(curr_instruction.left_operand)
                self.memory.set_value(curr_instruction.result, value)
                message = f"Assigning: {value} to {curr_instruction.result}"
                
            elif curr_operator in BINARY_OPS:
                fn, msg = BINARY_OPS[curr_operator]
                left_value = self.memory.get_value(curr_instruction.left_operand)
                right_value = self.memory.get_value(curr_instruction.right_operand)
                self.memory.set_value(curr_instruction.result, fn(left_value, right_value))
                message = msg(left_value, right_value)

            elif curr_operator == QuadCode.END:
                self.logger.info(curr_operator, "Program execution completed")
                break

            self.logger.info(curr_operator, message)

        # if self.logger.verbose:
        #     self.logger.print_logs()
