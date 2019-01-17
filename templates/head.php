<!doctype html>
<html>
    <head>
        <meta name="viewport" content="width=device-width"/>
        <meta charset="utf-8"/>
        <meta name="keywords" content="Churchill STEM Club,Winston Churchill STEM Club, WCHS STEM Club,STEM Club,Churchill High School STEM Club,Winston Churchill High School STEM Club,Churchill STEM"/>
        <?php
            if (isset($description) && !empty($description)) {
                echo "<meta name='description' content='$description'/>";
            }
        ?>
        
        <?php
            if (isset($title) && !empty($title)) {
                echo "<meta name='title' content='$title'/>";
            } else {
                echo '<meta name="title" content="Winston Churchill High School STEM Club"/>';
            }
        ?>

        <title><?php
            if (isset($title) && !empty($title)) {
                echo "$title | ";
            }
        ?>WCHS STEM Club</title>
        <link rel="stylesheet" href="<?php echo $path_to_root ?>reset.css"/>
        <link rel="stylesheet" href="<?php echo $path_to_root ?>style.css"/>
        <link rel="stylesheet" href="<?php echo $path_to_root ?>fonts.css"/>
        <link rel="icon" href="<?php echo $path_to_root ?>favicon.ico"/>
        <?php
            if (isset($extras) && !empty($extras)) {
                echo $extras;
            }
        ?>
    </head>
    <body>
        <header>
            <picture>
                <source type="image/webp" srcset="<?php echo $path_to_root ?>logo.webp"/>
                <img class="logo" src="<?php echo $path_to_root ?>logo.png" alt="STEM Club Logo"/>
            </picture>
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
