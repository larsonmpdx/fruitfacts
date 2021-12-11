# date formats

-   don't editorialize or interpret dates when copying them in from the source document. I want to preserve the original form
-   for example, don't take "mid september" and input it as "september 15" because that might lose some information
-   the goal is to accept a wide range of date formats as-is

# todo

-   script to beautify json
-   import script error detection:

    -   all files should be valid json
    -   no duplicate names (when considering name+category)

-   the oddball file lists category for each item. whenever category is omitted it's taken from the filename
