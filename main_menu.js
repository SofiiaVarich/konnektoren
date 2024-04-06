document.addEventListener('DOMContentLoaded', function() {
    const burger = document.querySelector('.burger-menu');
    const menuItems = document.querySelector('.menu-items');

    burger.addEventListener('click', function() {
        if (!menuItems.classList.contains('active')) {
            anime({
                targets: '.menu-items',
                translateX: ['100%', '0%'],
                opacity: [0, 1],
                easing: 'easeOutExpo',
                duration: 500
            });
        } else {
            anime({
                targets: '.menu-items',
                translateX: ['0%', '100%'],
                opacity: [1, 0],
                easing: 'easeInExpo',
                duration: 500
            });
        }
        menuItems.classList.toggle('active');
        burger.classList.toggle('active');
    });
});