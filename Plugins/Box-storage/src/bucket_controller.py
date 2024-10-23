import uuid
from bucket import Bucket
from typing import List, Dict
import pprint

println = pprint.PrettyPrinter(depth=3).pprint

class BucketController:
    def __init__(self: object) -> object:
        self.name = uuid.uuid4()
        self.buckets: Dict = {"Bucket Controller": {}}

    def __str__(self: object) -> str:
        return f"BUCKET MANAGER NAME {self.name}\nBUCKETS: {self.buckets}"

    def create_bucket(self: object, user_id: str):
        bucket_id = str(uuid.uuid4())
        user_id = str(user_id)
        
        self.buckets["Bucket Controller"].update(
                {
                    user_id: {
                        "bucket_id": bucket_id,
                        "bucket": Bucket(str(bucket_id))
                    }
                }
        )
        
    def get_buckets(self: object) -> str:
        return self.buckets

bc = BucketController()
bc.create_bucket(uuid.uuid4())
bc.create_bucket(uuid.uuid4())

println(bc.get_buckets())
