# date formats
* the goal is to accept a wide range of english day of year formats or ranges like "early to mid August".  ideally the format stored in these files should be the same as in the source document.  see the import code for allowed formats

# todo
* script to beautify json
* import script error detection:
  * all files should be valid json
  * no duplicate names (when considering name+category)

* the oddball file lists category for each item. whenever category is omitted it's taken from the filename
