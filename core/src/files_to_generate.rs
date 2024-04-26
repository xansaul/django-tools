use std::collections::HashMap;
use crate::utils;

pub struct SetViewFiles;
pub struct CommonFiles;
pub struct ApiViewFiles;

impl CommonFiles {
    
    
    pub fn files_no_variables() -> HashMap<&'static str, String> {
        let mut map = HashMap::new();
        map.insert("migrations/__init__.py", String::from(""));
        map.insert("__init__.py", String::from(""));
        map.insert("admin.py", String::from("from django.contrib import admin\n\n\n# Register your models here."));
        map.insert("test.py", String::from("from django.test import TestCase\n\n\n# Create your tests here."));
        map
    }
    
}

impl ApiViewFiles {
    pub fn files(name: &String) -> HashMap<&'static str, String> {
        let mut map = HashMap::new();
        let name_cap = utils::capitalize_first_letter(name);

        map.insert("models.py", format!("from django.db import models\n\n\n# Create your models here."));
       
        map.insert("urls.py", format!("from django.urls import path
from .views import {}Api


urlpatterns = [
    path('', {}Api.as_view())
]
            ", name_cap, name_cap));
        map.insert("views.py", format!("from rest_framework.views import APIView
from rest_framework.response import Response

class {}Api(APIView):

    def get(self, request):
        return Response({{
            'message': 'get {}'
        }})

    def post(self, request):
        return Response({{
            'message': 'post {}'
        }})

    def put(self, request):
        return Response({{
            'message': 'put {}'
        }})

    def delete(self, request):
        return Response({{
            'message': 'delete {}'
        }})

    def patch(self, request):
        return Response({{
            'message': 'patch {}'
        }})
            ", name_cap, name, name, name, name, name));
        map.insert("app.py", format!("from django.apps import AppConfig


class {}Config(AppConfig):
    default_auto_field = 'django.db.models.BigAutoField'
    name = '{}'

            ", name_cap, name));
        
        map
    }
}

impl SetViewFiles {

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
router.register(r'^/{}', {}Api)

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

