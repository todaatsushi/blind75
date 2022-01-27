from beartype import beartype

from mapper import PARENTHESIS_MAPPER


@beartype
def valid_parentheses(s: str) -> bool:
    stack = []
    for bracket in s:
        if PARENTHESIS_MAPPER.get(bracket):
            # bracket is closing
            if not stack:
                return False

            expecting = PARENTHESIS_MAPPER[bracket]
            last_bracket = stack.pop()
            if expecting != last_bracket:
                return False
        else:
            stack.append(bracket)
    return bool(stack) is False
