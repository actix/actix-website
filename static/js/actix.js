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
    } else {
      activateFeature('#' + $('div.actix-feature')[0].id);
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
