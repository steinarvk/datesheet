# datesheet

`datesheet` is a small web app to generate a PDF containing a table with all the
hours of a month on it, with one row per day and one column per hour. This sheet
can be printed and used to track events of interest using pen and paper.

This is a pretty trivial task, although useful to me. One might argue that it
doesn't actually need to be a web app, but it's 2022, what _isn't_ a web app at
this point?

Perhaps more interestingly, the app is meant to run very cheaply in a
scale-to-zero environment, i.e. near-zero cost for infrequent use (given the
nature of this app, it should ideally see one request per user per month). To
make this reasonably user-friendly, it's optimized for cold start time. To
achieve _that_ again, it's optimized to have a small container image.

Because it's fun to explore optimization techniques, I've tried out a bunch of
them to make the image as small as possible.

The app currently runs off a 1.2 MB image.
