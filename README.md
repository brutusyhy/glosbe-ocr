# Glosbe-ocr
A simple tool that captures the region around your cursor, extract word and look it up on `glosbe.com`

# Motivation
As a language enthusiast I often dreamed about a tool to easily look up a word without disrupting what you were doing at the moment. A couple of days ago I came across `Lookupper`, which I really enjoy. The premise is that it will identify the word you are pointing at, and create a popup window that displays its dictionary explanation. However, it is a proprietary software, so I decided to write an open-source alternative to it since it doesn't seem that hard to implement.

# Current capabilities
The app uses `tesseract` to do OCR. When you press ctrl-D while hovering over a word, the app takes a screenshot centered around your mouse cursor, runs tesseract to detect the word closest to your mouse, and look that word up on `glosbe.com`. For those who don't know, `glosbe.com` is one of the best free online dictionaries out there, with support for translating from and to pretty much any language, and which also provides real-world sentences and phrases that put the words in context. Since `glosbe.com` has long stopped providing API for external use, we are only using the publically available search functionality, and thus what this app does is not that different from you manually typing the word into the search box. In the future, I might incorporate more sources of explanation and things like pronunciation into the app.

# Limitations
The app is yet to support non-latin script, although that is totally possible. It also doesn't do much preprocessing of the word to be looked up, thus you might not get a good result for declined words. 

# Next step
-[] Caching of search results
-[] Lemmatization of search word
-[] More sources
