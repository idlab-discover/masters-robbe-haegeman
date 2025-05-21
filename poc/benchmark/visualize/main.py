from parsing import parse_results_file

if __name__ == "__main__":
    cases = parse_results_file("../result.jsonl")
    print(cases)
