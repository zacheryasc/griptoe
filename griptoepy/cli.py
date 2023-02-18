from argparse import ArgumentParser
from enum import Enum

import griptoepy.accounts

class Command(Enum):
    ACCOUNTS = "accounts"
    COSMOS = "cosmos"
    ETHEREUM = "ethereum"

def main():
    parser = ArgumentParser()
    parser.add_argument("command", type=Command, help="accounts cosmos ethereum")
    
    args = vars(parser.parse_args())
    if args['command'] == Command.ACCOUNTS:
        print("work")

if __name__ == "__main__":
    main()