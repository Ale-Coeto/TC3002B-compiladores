from src.quad_codes import QuadCode

class VirtualMachine:
    def __init__(self, program):
        self.instruction_pointer = 0
        self.back_to_pointer = 0
        self.program = program
        self.functions = {func.name: func for func in program.functions}
        self.constants = {const.address: const.value for const in program.constants}
        self.memory = {}
    
    def run(self):
        x = 0
        while True:
            curr_instruction = self.program.quads[self.instruction_pointer]
            curr_operator = curr_instruction.operator
            self.instruction_pointer += 1
            print(f"Executing: {curr_instruction.operator.name} {curr_instruction.left_operand} {curr_instruction.right_operand} {curr_instruction.result}")

            if curr_operator == QuadCode.GOTO:
                self.instruction_pointer = curr_instruction.result

            elif curr_operator == QuadCode.GOTOF:
                value = self.memory.get(curr_instruction.left_operand)
                if value is False:
                    self.instruction_pointer = curr_instruction.result

            elif curr_operator == QuadCode.GOSUB:
                self.back_to_pointer = self.instruction_pointer
                self.instruction_pointer = self.functions[curr_instruction.result].start_index

            elif curr_operator == QuadCode.ENDFUNC:
                self.instruction_pointer = self.back_to_pointer

            elif curr_operator == QuadCode.END:
                break
            x += 1