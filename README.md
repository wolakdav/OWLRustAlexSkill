# OWLRustProject

David Wolak
6/7/19
CS 410P

This projects goal was to create a lightweight way to check in on the current status of the Overwatch league with is a esports league. Their website is bulky and can be painful on a weaker connection, so instead I create a cli interface to figure out some basic information that I needed most of the time.

The way  it gets the information is though the undocmented overwatch league API which requires making HTTPS requests and then parsing the information as a JSON.

In order to build:
Just run cargo build, that's it! There are no speical instructions outside of this.

In order to ruin:
Just run cargo run! Then input the corresponding number with the selection you would like to choose.



