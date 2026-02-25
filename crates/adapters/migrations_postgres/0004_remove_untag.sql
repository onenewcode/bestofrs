DELETE FROM repo_tag_map
WHERE tag_id = 'tag:UNTAG:untagged';

DELETE FROM tags
WHERE id = 'tag:UNTAG:untagged'
   OR (LOWER(label) = LOWER('UNTAG') AND LOWER(value) = LOWER('untagged'));
