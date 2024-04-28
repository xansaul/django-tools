use std::collections::HashMap;
use cli_logic::utils;

pub struct ViewSetFiles;

impl ViewSetFiles {

    pub fn files(name: &String) -> HashMap<&'static str, String> {
        let mut map = HashMap::new();
        let name_cap = utils::capitalize_first_letter(name);
        let name_without_last_character_cap = utils::remove_last_character(&name_cap);
    
        map.insert("models.py", format!("from django.db import models\n\nclass {}(models.Model):\n\tpass", name_without_last_character_cap));
        map.insert("serializers.py", format!("from rest_framework.serializers import ModelSerializer
from .models import {}


class {}Serializer(ModelSerializer):
    class Meta:
        model = {}
        fields = '__all__'"
            , name_without_last_character_cap, name_without_last_character_cap, name_without_last_character_cap));
        map.insert("urls.py", format!("from rest_framework import routers
from .views import {}Api

router = routers.SimpleRouter()
router.register(r'{}', {}Api)

urlpatterns = router.urls
            ", name_cap, name, name_cap));
        map.insert("views.py", format!("from rest_framework.viewsets import ModelViewSet
from .models import {}
from .serializers import {}Serializer


class {}Api(ModelViewSet):
    queryset = {}.objects.all()
    serializer_class = {}Serializer
    
            ", name_without_last_character_cap, name_without_last_character_cap, name_cap, name_without_last_character_cap, name_without_last_character_cap));
        map.insert("app.py", format!("from django.apps import AppConfig
    

class {}Config(AppConfig):
    default_auto_field = 'django.db.models.BigAutoField'
    name = '{}'
    
            ", name_cap, name));
        
        map
    }
}

