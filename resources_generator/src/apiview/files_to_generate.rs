use std::collections::HashMap;
use cli_logic::utils;


pub struct ApiViewFiles;


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

