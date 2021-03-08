---
layout: default
title: About Us
---
# Who makes BrightShard's Apps?

I'm not the only person behind these apps. There are others who help me too!

{% for author in site.authors %}
# [{{author.name}}]({{author.url}})
### {{author.position}}
{{author.content | markdownify}}
{% endfor %}
