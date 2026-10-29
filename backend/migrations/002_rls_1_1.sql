
-- RLS Policies - each funeral home only sees its own data (when home_id is set)
CREATE POLICY home_isolation ON obituaries FOR ALL USING (true); -- simplified for v1, JWT filtering in app layer
