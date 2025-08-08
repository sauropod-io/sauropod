WITH RECURSIVE chain(
  response_id,
  parent_response_id,
  response_request,
  response_output,
  user_id,
  depth
) AS (
  -- base row (must belong to the user)
  SELECT r.response_id,
         r.parent_response_id,
         r.response_request,
         r.response_output,
         r.user_id,
         0
  FROM response r
  WHERE r.response_id = ?1
    AND r.user_id = ?2

  UNION ALL

  -- walk up parents (also constrained to same user)
  SELECT p.response_id,
         p.parent_response_id,
         p.response_request,
         p.response_output,
         p.user_id,
         chain.depth + 1
  FROM response p
  JOIN chain ON chain.parent_response_id = p.response_id
  WHERE p.user_id = ?2
)
SELECT response_id, response_request, response_output, depth AS "depth: i64"
FROM chain
ORDER BY depth DESC; -- closest parent first
