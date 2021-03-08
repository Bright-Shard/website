---
layout: default
title: Blog
---
# Latest Blog Posts
{% for post in site.posts %}
## [{{ post.title | markdonify }}]({{ post.url }})
{{ post.excerpt | markdownify }}
{% endfor %}
