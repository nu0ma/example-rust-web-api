\c example;
CREATE TABLE organization (
  id UUID NOT NULL,
  name text NOT NULL,
  PRIMARY KEY (id)
);
CREATE TABLE member (
  id UUID NOT NULL,
  name text NOT NULL,
  organization_id UUID NOT NULL REFERENCES organization(id),
  PRIMARY KEY (id)
);