window.onload = function(){
    if (window.location.href.search("cn") == -1) {
        document.getElementById("nav-blog").style.display = "none"
    }
}

(function() {
  function activateFeature(sel) {
    $('div.actix-feature').hide();
    $(sel).show();
    $('li.actix-feature-selector').removeClass('active');
    $('li.actix-feature-selector > a').each(function() {
      if (this.getAttribute('href') === sel) {
        $(this).parent().addClass('active');
      }
    });
  }

  function initFeatureSelector() {
    $('div.actix-feature').hide();
    var active = $(window.location.hash);
    if (active.is('div.actix-feature')) {
      activateFeature(window.location.hash);
      $('html, body').animate({
        scrollTop: $('.actix-showcase').offset().top
      }, 1000);
    } else {
      var firstFeature = $('div.actix-feature')[0];
      if (firstFeature) {
        activateFeature('#' + firstFeature.id);
      }
    }

    $('ul li.actix-feature-selector a').on('click', function(evt) {
      evt.preventDefault();
      history.replaceState({}, '', evt.target.href);
      activateFeature(this.getAttribute('href'));
    });
  }

  $(function() {
    initFeatureSelector();
  });
})();
