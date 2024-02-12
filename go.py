import os


def get_paths(path):
    return os.listdir(path)


def load_file(path):
    # load the file as a string
    with open(path, 'r') as f:
        file = f.read()
    return file


map = {
    'a': 0,
    'b': 1,
    'c': 2,
    'd': 3,
    'e': 4,
    'f': 5,
    'g': 6,
    'h': 7,
    'i': 9,
}

BLACK = 1
WHITE = -1
EMPTY = 0
SIZE = 9


def get_moves(file):

    idx = 0
    moves = []

    while idx < len(file)-1:

        if file[idx].lower() in ["b", "w"] and file[idx+1] == "[":
            
            move = []
            if file[idx+2].lower() not in map or file[idx+3].lower() not in map:
                idx += 1
                continue

            move.append(map[file[idx+2].lower()])
            move.append(map[file[idx+3].lower()])
            moves.append(move)

        idx += 1

    return moves


def is_valid(x, y, board, color):
    
    board[y][x] = color

    def get_liberties(x, y, color):

        if x >= 9 or x < 0:
            return False
        
        if y >= 9 or y < 0:
            return False
        
        if [x, y] in visited:
            return False
        
        visited.add([x, y])

        if board[y][x] == EMPTY:
            return True
        
        elif board[y][x] != color:
            return False
        
        if get_liberties(x+1, y, color) or get_liberties(x-1, y, color) or get_liberties(x, y+1, color) or get_liberties(x, y-1, color):
            return True

        return False
    
    visited = set()
    if not get_liberties(x+1, y, color*-1) and len(visited) > 0:
        board[y][x] = EMPTY
        return True
    
    visited = set()
    if not get_liberties(x-1, y, color*-1) and len(visited) > 0:
        board[y][x] = EMPTY
        return True
    
    visited = set()
    if not get_liberties(x, y+1, color*-1) and len(visited) > 0:
        board[y][x] = EMPTY
        return True
    
    visited = set()
    if not get_liberties(x, y-1, color*-1) and len(visited) > 0:
        board[y][x] = EMPTY
        return True
    
    # see if color has empties
    visited = set()
    has_liberties = get_liberties(x, y, color)
    if has_liberties:
        board[y][x] = EMPTY
        return True
    
    return False


        
def place(x, y, board, color):
    board[y][x] = color

    def get_liberties(x, y, color):

        if x >= 9 or x < 0:
            return False
        
        if y >= 9 or y < 0:
            return False
        
        if [x, y] in visited:
            return False
        
        visited.add([x, y])

        if board[y][x] == EMPTY:
            return True
        
        elif board[y][x] != color:
            return False
        
        if get_liberties(x+1, y, color) or get_liberties(x-1, y, color) or get_liberties(x, y+1, color) or get_liberties(x, y-1, color):
            return True

        return False
    
    visited = set()
    if not get_liberties(x+1, y, color*-1) and len(visited) > 0:
        return True
    
    visited = set()
    if not get_liberties(x-1, y, color*-1):
        for pos in visited:
            
    
    visited = set()
    get_liberties(x, y+1, color*-1) 
    visited = set()
    get_liberties(x, y-1, color*-1)
    
    # see if color has empties
    visited = set()
    get_liberties(x, y, color)
    

    
    return board




def generate_states(moves):
    
    board = [[0 for i in range(9)] for j in range(9)]

    turn = BLACK

    for move in moves:

        y = move[1]
        x = move[0]

        if x >= 9 or x < 0:
            return False
        
        if y >= 9 or y < 0:
            return False

        # if already taken, return False
        if board[y][x] != 0:
            return False
        
        board[y][x] = turn
        
        if turn == BLACK:
            turn = WHITE
        else:
            turn = BLACK



def main():
    files = get_paths('data')
    for file in files:
        file_path = os.path.join('data', file)
        file = load_file(file_path)
        moves = get_moves(file)
        states = generate_states(moves)
    



if __name__ == '__main__':
    main()
