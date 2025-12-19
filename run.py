import os
import subprocess
import time
import json
from pathlib import Path

def count_lines(file_path):
    # LOC
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            return sum(1 for _ in f)
    except Exception as e:
        return None

def run_rust_files(directory_path, output_filename="results.json"):
    dir_path = Path(directory_path)
    if not dir_path.is_dir():
        print(f"Error: {directory_path} is not a valid directory.")
        return

    results = []

    print(f"Processing Rust files in: {dir_path.absolute()}")

    for file in dir_path.glob("*.rs"):
        file_name = file.name
        temp_binary = file.stem 
        
        loc = count_lines(file)

        compile_process = subprocess.run(
            ["rustc", str(file), "-o", temp_binary],
            capture_output=True,
            text=True
        )

        if compile_process.returncode != 0:
            print(f"Skipping {file_name}: Compilation Failed")
            continue

        start_time = time.perf_counter()
        
        binary_cmd = f"./{temp_binary}" if os.name != 'nt' else f"{temp_binary}.exe"
        
        try:
            subprocess.run([binary_cmd], capture_output=True, text=True, check=True)
            end_time = time.perf_counter()
            exec_duration = end_time - start_time
            
            results.append({
                "filename": file_name,
                "loc": loc,
                "execution_time_seconds": round(exec_duration, 6)
            })
            print(f"Processed {file_name}")

        except subprocess.CalledProcessError:
            print(f"Skipping {file_name}: Runtime Error")
        
        finally:
            if os.path.exists(binary_cmd):
                os.remove(binary_cmd)

    with open(output_filename, 'w', encoding='utf-8') as jf:
        json.dump(results, jf, indent=4)
    
    print(f"\nSuccess! Results saved to {output_filename}")

if __name__ == "__main__":
    # TODO change with actual directory 
    run_rust_files(".") 