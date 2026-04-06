// Shared nav loader — single source of truth for navigation
(function() {
    var nav = document.getElementById('main-nav');
    if (!nav) return;
    fetch('nav.html')
        .then(function(r) { return r.text(); })
        .then(function(html) {
            nav.innerHTML = html;
            // Highlight active page
            var page = location.pathname.split('/').pop() || 'index.html';
            var links = nav.querySelectorAll('a');
            for (var i = 0; i < links.length; i++) {
                if (links[i].getAttribute('href') === page) {
                    links[i].classList.add('active');
                }
            }
        });
})();
