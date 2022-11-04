\c example;
CREATE TABLE organization (
  id INT NOT NULL,
  name text NOT NULL,
  PRIMARY KEY (id)
);
CREATE TABLE member (
  id INT NOT NULL,
  name text NOT NULL,
  organization_id INT NOT NULL REFERENCES organization(id),
  PRIMARY KEY (id)
);