<!doctype html>
<html>
    <head>
        <meta name="viewport" content="width=device-width"/>
        <title>
        <?php
            if (isset($title) && !empty($title)) {
                echo $title . " | ";
            }
        ?>WCHS STEM Club</title>
        <link rel="stylesheet" href="<?php echo $path_to_root ?>reset.css"/>
        <link rel="stylesheet" href="<?php echo $path_to_root ?>style.css"/>
        <link rel="stylesheet" media="screen" href="https://fontlibrary.org/face/bebusneuebold" type="text/css"/>
        <link rel="stylesheet" media="screen" href="https://fontlibrary.org/face/gidole-regular" type="text/css"/> 
        <?php
            if (isset($extras) && !empty($extras)) {
                echo $extras;
            }
        ?>
    </head>
    <body>
        <header>
            <h1><a href="<?php echo $path_to_root ?>">WCHS STEM Club</a></h1>
            <nav>
                <ul>
                    <li><a href="<?php echo $path_to_root ?>">Home</a></li>
                    <li><a href="<?php echo $path_to_root ?>#about">About</a></li>
                    <li><a href="<?php echo $path_to_root ?>sponsors">Sponsors</a></li>
                    <li><a href="<?php echo $path_to_root ?>outreach">Our outreach</a></li>
                    <li><a href="<?php echo $path_to_root ?>contact">Contact us</a></li>
                </ul>
            </nav>            
        </header>