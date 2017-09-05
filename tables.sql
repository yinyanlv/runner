# 用户表
DROP TABLE user;
CREATE TABLE user (
    id int(16) AUTO_INCREMENT NOT NULL,
    username varchar(32) COLLATE utf8mb4_unicode_ci NOT NULL,
    sex int(1) DEFAULT 1,
    email varchar(64) COLLATE utf8mb4_unicode_ci NOT NULL,
    password varchar(32) COLLATE utf8mb4_unicode_ci NOT NULL,
    salt varchar(32) COLLATE utf8mb4_unicode_ci NOT NULL,
    create_time datetime NOT NULL,
    PRIMARY KEY (id),
    UNIQUE KEY username (username),
    UNIQUE KEY email (email)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

# 文章表
DROP TABLE article;
CREATE TABLE article (
    id int(32) AUTO_INCREMENT NOT NULL,
    user_id int(16) NOT NULL,    
    category varchar(32) NOT NULL,
    title varchar(64) COLLATE utf8mb4_unicode_ci NOT NULL,
    content mediumtext COLLATE utf8mb4_unicode_ci NOT NULL,
    status varchar(8) NOT NULL,
    priority tinyint(4) DEFAULT 0,    
    comment_count int(16) DEFAULT 0,
    create_time datetime NOT NULL,
    update_time datetime NOT NULL,   
    PRIMARY KEY (id),
    KEY user_id (user_id),
    CONSTRAINT article_ibfk_1 FOREIGN KEY (user_id) REFERENCES user (id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

# 评论表
DROP TABLE comment;
CREATE TABLE commnet (
    id int(32) AUTO_INCREMENT NOT NULL,
    article_id int(32) NOT NULL,
    user_id int(16) NOT NULL,
    content mediumtext COLLATE utf8mb4_unicode_ci NOT NULL,
    create_time datetime NOT NULL,
    PRIMARY KEY (id),
    KEY article_id (article_id),
    KEY user_id (user_id),
    CONSTRAINT comment_ibfk_1 FOREIGN KEY (article_id) REFERENCES article (id), 
    CONSTRAINT comment_ibfk_2 FOREIGN KEY (user_id) REFERENCES user (id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

# 消息表
DROP TABLE message;
CREATE TABLE message (
    id int(32) AUTO_INCREMENT NOT NULL,
    article_id int(32) NOT NULL,
    comment_id int(32) NOT NULL,
    from_user_id int(16) NOT NULL,
    to_user_id int(16) NOT NULL,
    content mediumtext COLLATE utf8mb4_unicode_ci NOT NULL,
    create_time datetime NOT NULL,
    status varchar(8) NOT NULL,
    PRIMARY KEY (id),
    KEY article_id (article_id),
    KEY comment_id (comment_id),
    KEY from_user_id (from_user_id),
    KEY to_user_id (to_user_id),
    CONSTRAINT message_ibfk_1 FOREIGN KEY (article_id) REFERENCES article (id),
    CONSTRAINT message_ibfk_2 FOREIGN KEY (comment_id) REFERENCES comment (id),
    CONSTRAINT message_ibfk_3 FOREIGN KEY (from_user_id) REFERENCES user (id),
    CONSTRAINT message_ibfk_4 FOREIGN KEY (to_user_id) REFERENCES user (id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;



