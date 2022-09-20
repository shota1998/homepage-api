CREATE TABLE editing_articles (
  id SERIAL,
  article_id INT NOT NULL,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  PRIMARY KEY (id),
  FOREIGN KEY (article_id) REFERENCES articles(id)
);
