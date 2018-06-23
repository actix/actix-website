window.onload = function(){
  if (window.location.href.search("cn") != -1) {
      var actix_home = document.getElementById("act-home")
      actix_home.style.display = "none"
  }else{
      var actix_home_cn = document.getElementById("act-home-cn")
      actix_home_cn.style.display = "none"
  }
}

function setTab(name,cursel){
  let tlinks = document.getElementById("act-cn-tabs").getElementsByTagName('li')
  for(var i=1; i<=tlinks.length; i++){
      var menu = document.getElementById(name+i);
      var menudiv = document.getElementById("con_"+name+"_"+i);
      if(i==cursel){
          menu.className="off";
          menudiv.style.display="block";
      }
      else{
          menu.className="";
          menudiv.style.display="none";
      }
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


