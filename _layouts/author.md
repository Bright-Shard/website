---
layout: default
---
# {{page.name}}
## {{page.position}}

{{ content }}

## Posts by {{page.name}}
{% assign filtered_posts = site.posts | where: 'author', page.short_name %}
{% for post in filtered_posts %}
[{{post.title}}]({{post.url}})
{% endfor %}
