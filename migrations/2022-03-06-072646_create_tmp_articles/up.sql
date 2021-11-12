CREATE TABLE tmp_articles (
  id SERIAL,
  articl_id INT NOT NULL,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  PRIMARY KEY (id)
);
