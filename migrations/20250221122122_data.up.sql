-- Add up migration script here
INSERT INTO
    guild
VALUES
    (1056217255307919370);

INSERT INTO
    logs_settings
VALUES
    (1056217255307919370, true, 1252535080048656435);

INSERT INTO
    tickets_settings
VALUES
    (1056217255307919370, true, 1252535080048656435);

INSERT INTO
    ticket (guild_id, author_id, title, info)
VALUES
    (
        1056217255307919370,
        255251641144442880,
        'test 1',
        'foobar'
    ),
    (
        1056217255307919370,
        255251641144442880,
        'test 2',
        'foobar'
    ),
    (
        1056217255307919370,
        255251641144442880,
        'test 3',
        'foobar'
    ),
    (
        1056217255307919370,
        255251641144442880,
        'test 4',
        'foobar'
    ),
    (
        1056217255307919370,
        255251641144442880,
        'test 5',
        'foobar'
    );

INSERT INTO
    ticket (guild_id, author_id, title, info)
VALUES
    (
        1056217255307919370,
        260997919144935426,
        'test 1231412341',
        'foobar'
    ),
    (
        1056217255307919370,
        260997919144935426,
        'test 2462345623',
        'foobar'
    ),
    (
        1056217255307919370,
        260997919144935426,
        'test 3563456345',
        'foobar'
    ),
    (
        1056217255307919370,
        260997919144935426,
        'test 4564564',
        'foobar'
    ),
    (
        1056217255307919370,
        260997919144935426,
        'test 345634565',
        'foobar'
    );

INSERT INTO
    warn (guild_id, staff_member_id, target_user_id, reason)
VALUES
    (
        1056217255307919370,
        255251641144442880,
        255251641144442880,
        'foobar'
    );
