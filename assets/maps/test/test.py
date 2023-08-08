import re

def read_instructions(file_path):
    instructions = []
    
    with open(file_path, 'r') as file:
        data = file.read()
        matches = re.findall(r'\((.*?)\)(\w{4})', data)
        
        for match in matches:
            instruction = match[0]
            tile_id = match[1]
            instructions.append((instruction, tile_id))
    
    return instructions

file_path = '/home/malkmusl/dev/rust/learn-bevy/assets/maps/test/0.map'
instructions = read_instructions(file_path)

for instruction, tile_id in instructions:
    print(f"Instruction: {instruction}, Tile ID: {tile_id}")
