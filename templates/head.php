<!doctype html>
<html>
    <head>
        <meta name="viewport" content="width=device-width"/>
        <meta charset="utf-8"/>
        <?php
            if (isset($description) && !empty($description)) {
                echo "<meta name='description' content='$description'/>";
            }
        ?>
        
        <?php
            if (isset($title) && !empty($title)) {
                echo "<meta name='title' content='$title'/>";
            }
        ?>

        <title>
        <?php
            if (isset($title) && !empty($title)) {
                echo "$title | ";
            }
        ?>
        WCHS STEM Club</title>
        <link rel="stylesheet" href="<?php echo $path_to_root ?>reset.css"/>
        <link rel="stylesheet" href="<?php echo $path_to_root ?>style.css"/>
        <link rel="stylesheet" media="screen" href="https://fontlibrary.org/face/bebusneuebold" type="text/css"/>
        <link rel="stylesheet" media="screen" href="https://fontlibrary.org/face/gidole-regular" type="text/css"/>
        <link rel="icon" href="<?php echo $path_to_root ?>favicon.png"/>
        <?php
            if (isset($extras) && !empty($extras)) {
                echo $extras;
            }
        ?>
    </head>
    <body>
        <header>
            <img class="logo" src="<?php echo $path_to_root ?>example_logo.svg" alt="Logo"/>
            <h1><a href="<?php echo $path_to_root ?>">WCHS STEM Club</a></h1>
            <nav>
                <ul>
                    <li><a href="<?php echo $path_to_root ?>">Home</a></li>
                    <li><a href="<?php echo $path_to_root ?>#about">About</a></li>
                    <li><a href="<?php echo $path_to_root ?>sponsors">Sponsors</a></li>
                    <li><a href="<?php echo $path_to_root ?>outreach">Outreach</a></li>
                    <li><a href="<?php echo $path_to_root ?>contact">Contact</a></li>
                    <li><a href="<?php echo $path_to_root ?>classes">Classes</a></li>
                </ul>
            </nav>            
        </header>
