from src.quad_codes import QuadCode
        
BLUE = "\033[94m"
CYAN = "\033[96m"
GREEN = "\033[92m"
YELLOW = "\033[93m"
RESET = "\033[0m"

class Logger():
    def __init__(self, verbose=False, header_color=BLUE, title_color=CYAN):
        self.verbose = verbose
        self.header_color = header_color
        self.title_color = title_color
        self.logs = []

    def header(self, title, message):
        res = f"{self.header_color}=== {title} === -> {message}{RESET}"
        self.logs.append(res)
        if self.verbose:
            print(res)

    def _operator_color(self, operator):
        if operator in (QuadCode.ERA, QuadCode.PARAM, QuadCode.GOSUB, QuadCode.ENDFUNC):
            return CYAN  # function
        if operator in (QuadCode.ASSIGN,QuadCode.ADD, QuadCode.SUB, QuadCode.MUL, QuadCode.DIV,
                        QuadCode.LT, QuadCode.GT, QuadCode.EQ, QuadCode.NEQ):
            return GREEN   # arithmetic / comparison
        if operator in (QuadCode.GOTO, QuadCode.GOTOF, QuadCode.GOTOT):
            return YELLOW   # — jumps
        return self.title_color

    def info(self, operator, message):
        color = self._operator_color(operator)
        res = f"[{color}{operator.name}{RESET}] {message}"
        self.logs.append(res)
        if self.verbose:
            print(res)

    def print_logs(self):
        for log in self.logs:
            print(log)