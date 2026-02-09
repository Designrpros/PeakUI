
import sys

def read_leb128(data, offset):
    result = 0
    shift = 0
    while True:
        byte = data[offset]
        offset += 1
        result |= (byte & 0x7f) << shift
        if not (byte & 0x80):
            break
        shift += 7
    return result, offset

def read_name(data, offset):
    length, offset = read_leb128(data, offset)
    name = data[offset:offset+length].decode('utf-8')
    offset += length
    return name, offset

def parse_wasm(file_path):
    with open(file_path, 'rb') as f:
        data = f.read()

    if data[0:4] != b'\0asm':
        print("Not a WASM file")
        return

    offset = 8
    while offset < len(data):
        section_id = data[offset]
        offset += 1
        payload_len, offset = read_leb128(data, offset)
        
        next_section_offset = offset + payload_len

        if section_id == 2: # Import Section
            print("Found Import Section, parsing...")
            count, offset = read_leb128(data, offset)
            print(f"Total imports: {count}")
            
            for i in range(count):
                module_name, offset = read_name(data, offset)
                field_name, offset = read_name(data, offset)
                kind = data[offset]
                offset += 1
                
                # kind: 0=func(typeidx), 1=table(tabletype), 2=mem(memtype), 3=global(globaltype)
                type_desc = ""
                if kind == 0:
                    type_idx, offset = read_leb128(data, offset)
                    type_desc = f"func[{type_idx}]"
                elif kind == 1:
                    # tabletype: reftype, limits
                    reftype = data[offset]
                    offset += 1
                    flags, offset = read_leb128(data, offset)
                    min_val, offset = read_leb128(data, offset)
                    if flags & 1:
                        max_val, offset = read_leb128(data, offset)
                    type_desc = "table"
                elif kind == 2:
                    # memtype: limits
                    flags, offset = read_leb128(data, offset)
                    min_val, offset = read_leb128(data, offset)
                    if flags & 1:
                        max_val, offset = read_leb128(data, offset)
                    type_desc = "memory"
                elif kind == 3:
                    # globaltype: valtype, mut
                    offset += 2 # valtype + mut
                    type_desc = "global"

                if module_name == "env":
                    print(f"IMPORT: {module_name}.{field_name} ({type_desc})")
        
        offset = next_section_offset

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python inspect_wasm.py <wasm_file>")
    else:
        parse_wasm(sys.argv[1])
