var menu = document.getElementById('menu');
menu.addEventListener('click', function() {
    var nav = document.getElementById('rnav');
        if (nav.style.height == 'auto') {
            nav.style.height = '0';
        }else{
            nav.style.height = 'auto';
        }

        // they are same
        // manv.style.height = 'auto';
        // manv.setAttribute('style', 'height: auto !important');
        // manv.style.setProperty( 'height',' auto', 'important');
}, false);