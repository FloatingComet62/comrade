file = open("stress.cmr", "w")

file.write("""include std->io

fun main() => i32 {""")

for i in range(1000):
    file.write(f"""
    if {i} > 500 {{
        io->out({i})
    }}""")

file.write("""
}
""")

file.close()