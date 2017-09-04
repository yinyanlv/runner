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
    PRIMARY KEY (`id`),
    UNIQUE KEY (`username`),
    UNIQUE KEY (`email`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

# 文章表
DROP TABLE article;
CREATE TABLE article (
    id int(32) AUTO_INCREMENT NOT NULL,
    use_id int(16) NOT NULL,    
    category varchar(32) NOT NULL,
    title varchar(64) COLLATE utf8mb4_unicode_ci NOT NULL,
    content mediumtext COLLATE utf8mb4_unicode_ci NOT NULL,
    status varchar(8) NOT NULL,
    priority tinyint(4) DEFAULT 0,    
    comment_count int(16) DEFAULT 0,
    create_time datetime NOT NULL,
    update_time datetime NOT NULL,   
    PRIMARY KEY (`id`),
    CONSTRAINT `article_ibfk_1`  FOREIGN KEY (`use_id`) REFERENCES `user` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;



