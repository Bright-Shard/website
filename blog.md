---
layout: default
title: Blog
---
# Latest Blog Posts
{% for post in site.posts %}
## [{{ post.title}} ]({{ post.url}})
{{ post.excerpt }}
{% endfor %}
