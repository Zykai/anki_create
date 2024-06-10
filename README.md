# Automatic anki card creation for Chinese
## Setup
* Download the [CC-EDICT]([CC-EDICT](https://www.mdbg.net/chinese/dictionary?page=cedict)) dictionary file (`cedict_ts.u8`) and add it to the `res` folder
* Optional: Get a tsv-File for your known words by exporting from anki. The forth column needs to be the Chinese word
## Usage
* Run the program with the name of the book as a parameters (needs to be .txt file, no file extension
  * e.g. `cargo run 诡秘之主`
* Retrieve the generated file from the ``res``-Folder
* Import the deck to Anki using ``File->Import``
