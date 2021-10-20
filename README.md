# minimax-tictactoe

Playable tic-tac-toe engine using α-β pruning.

---

### Installation

If you are on windows, download the files [here](https://github.com/Emik03/minimax-tictactoe/releases/latest).

For mac/linux users, you need to compile this yourself.

---

### Contribution

As with most [GitHub](https://github.com/) repositories, you can contribute by:
* [Creating an issue](https://github.com/Emik03/minimax-tictactoe/issues) to either address bugs, feature suggestions, or other changes.
* [Creating or reviewing pull requests](https://github.com/Emik03/minimax-tictactoe/pulls) are also welcome.

---

### Building

1. [Install Rust](https://www.rust-lang.org/tools/install) if you don't already have it.

2. Download and extract the source code, or clone the repository:

```bash
git clone https://github.com/Emik03/minimax-tictactoe.git
```

3. In the root directory, run the following from a command-line:

```bash
cargo build --release
```

4. Find the binary file in `\target\release`.

---

### License

This project falls under the [GPLv3 License](https://github.com/Emik03/minimax-tictactoe/blob/main/LICENSE.md), which in-short means that you copy, modify, and redistribute under private and commerical use, but the author cannot be held liable, the license must stay intact as is, a disclosure of the original source has to be made, and you must state changes to this repository.

---

### Explanation

I was mainly driven by [DDD (Domain-Driven Design)](https://en.wikipedia.org/wiki/Domain-driven_design) to make a ton of abstractions and encapsulations. Therefore, a lot of the code is hidden away at other files so you don't have to worry about how they work.

[The entry point of the application](https://github.com/Emik03/minimax-tictactoe/blob/master/src/main.rs#L16) will print to the console and ask the user for a query. The [`of_input`](https://github.com/Emik03/minimax-tictactoe/blob/master/src/main.rs#L113) method in particular is useful, as it allows me to create a closure that does a series of tests on the provided `String`.

If this method returns `None` then we know something went wrong and we need to try again, if `Some(x)` is returned then we can use that as the actual return and the program continues.

After the user inputs everything, [all of the parameters get put into a wrapper method](https://github.com/Emik03/minimax-tictactoe/blob/master/src/evaluation.rs#L21)), and it is [the inner method call](https://github.com/Emik03/minimax-tictactoe/blob/master/src/evaluation.rs#L25) that starts the real core of the application.

Minimax is a brute-force algorithm that determines the best move by recursively playing each move to see how they play out. The idea is that we have a maximizing player, which tries to get the maximum number because it implies that they win, and a minimizing player, which tries to get the minimum number because it implies that they win. Think of it like tug-of-war where both sides are trying to pull the rope through.

We write an [evaluator method](https://github.com/Emik03/minimax-tictactoe/blob/master/src/board.rs#L102) that looks at the position and returns the number indicating who is winning. 0 means a dead draw, positive means the maximizing player is winning, and negative means the minimizing player is winning. To make it more effective, we also subtract the number of moves played, and this is to encourage moves that win quickly.

The first thing we do is check [whether the game has ended](https://github.com/Emik03/minimax-tictactoe/blob/master/src/evaluation.rs#L59), if the game is over then we should prematurely return the result of the game.

If the game goes on, we then [check the depth](https://github.com/Emik03/minimax-tictactoe/blob/master/src/evaluation.rs#L65) to see if it is now 0. The depth calculates how deep the algorithm goes and therefore how fast it will perform. A higher depth *usually* means better moves are played.

If we reach here, we will [start creating a vector for each evaluation](https://github.com/Emik03/minimax-tictactoe/blob/master/src/evaluation.rs#L72) which is needed later.

We then create a [loop of every theoretical possible move](https://github.com/Emik03/minimax-tictactoe/blob/master/src/evaluation.rs#L75). Some of the moves are illegal though, so we have a [move legality checker](https://github.com/Emik03/minimax-tictactoe/blob/master/src/evaluation.rs#L76) to return a new board state. We need to clone the board so that the original doesn't get mutated.

We then [call the inner minimax method](https://github.com/Emik03/minimax-tictactoe/blob/master/src/evaluation.rs#L78) again which makes it play a move deeper. We need to invert the `is_o` parameter so that the other player plays, and we decrement the `depth` value by 1 since it goes 1 method call deeper.

After that inner method call ends, we [add the result to the vector](https://github.com/Emik03/minimax-tictactoe/blob/master/src/evaluation.rs#L85) mentioned prior.

Finally, we [return the best move](https://github.com/Emik03/minimax-tictactoe/blob/master/src/evaluation.rs#L91) by getting the maximum evaluation if the maximizing player is playing, or the minimum evaluation if the minimizing player is playing.

While it takes more memory, we can make it more efficient by implementing α-β (alpha-beta) pruning to make the algorithm faster. The idea is that [each game stores an α value and a β value](https://github.com/Emik03/minimax-tictactoe/blob/master/src/board.rs#L13). α starts out as the most negative number, since it acconuts for the worst case scenario for the maximizing player, and β starts out as the most positive number, since it accounts for the worst case scenario for the minimizing player.

We then [check if the result of the current game exceeds α or β](https://github.com/Emik03/minimax-tictactoe/blob/master/src/evaluation.rs#L79). In the event that it does, it means that we don't need to search this branch and can therefore prune it.

---

### Credit

Thanks to the [`ptree`](https://docs.rs/ptree/0.3.2/ptree/) crate for the pretty tree generator.

Borrowed code from myself in [1D Chess](https://github.com/Emik03/EmikModules/tree/main/Assets/Modules/1D%20Chess).

Other code borrowed from the following sources:
- [How do I subtract one character from another in rust?](https://stackoverflow.com/questions/54583399/how-do-i-subtract-one-character-from-another-in-rust)
- [Convert a string to int](https://stackoverflow.com/questions/27043268/convert-a-string-to-int)
- [How to get elements from a vector with a computed index?](https://www.reddit.com/r/rust/comments/385em5/how_to_get_elements_from_a_vector_with_a_computed/)
- [std::option](https://doc.rust-lang.org/std/option/)
- [Proper way to return a new string in rust](https://stackoverflow.com/questions/43079077/proper-way-to-return-a-new-string-in-rust)
- [How do I store a closure in a struct in rust?](https://stackoverflow.com/questions/27831944/how-do-i-store-a-closure-in-a-struct-in-rust)
- [How to match against multiple characters in a rust match expression?](https://stackoverflow.com/questions/60102442/how-to-match-against-multiple-characters-in-a-rust-match-expression)
